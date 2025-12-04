package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type chatRequest struct {
	Model       string    `json:"model"`
	Messages    []message `json:"messages"`
	Temperature float32   `json:"temperature,omitempty"`
}

type chatResponse struct {
	Choices []struct {
		Message message `json:"message"`
	} `json:"choices"`
	Error *struct {
		Message string `json:"message"`
	} `json:"error,omitempty"`
}

func main() {
	defaultPrompt := filepath.Clean(filepath.Join("..", "agent-prompt.md"))
	promptPath := flag.String("prompt", defaultPrompt, "path to system prompt markdown")
	model := flag.String("model", envOr("OPENAI_MODEL", "gpt-4o-mini"), "OpenAI model name")
	apiURL := flag.String("url", envOr("OPENAI_API_BASE", "https://api.openai.com/v1/chat/completions"), "OpenAI chat completions endpoint")
	flag.Parse()

	apiKey := os.Getenv("OPENAI_API_KEY")
	if apiKey == "" {
		fmt.Fprintln(os.Stderr, "missing OPENAI_API_KEY")
		os.Exit(1)
	}

	systemPrompt, err := os.ReadFile(*promptPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to read prompt file (%s): %v\n", *promptPath, err)
		os.Exit(1)
	}

	fmt.Printf("Loaded system prompt from %s\n", *promptPath)
	fmt.Println("Type :q 退出；按 Enter 送出。")

	history := []message{
		{Role: "system", Content: string(systemPrompt)},
	}

	reader := bufio.NewReader(os.Stdin)
	for {
		fmt.Print("You> ")
		text, err := reader.ReadString('\n')
		if err != nil && err != io.EOF {
			fmt.Fprintf(os.Stderr, "read error: %v\n", err)
			continue
		}
		text = strings.TrimSpace(text)
		if text == "" && err == io.EOF {
			break
		}
		if text == "" {
			continue
		}
		if strings.EqualFold(text, ":q") {
			break
		}

		history = append(history, message{Role: "user", Content: text})
		reply, err := callOpenAI(*apiURL, apiKey, *model, history)
		if err != nil {
			fmt.Fprintf(os.Stderr, "API error: %v\n", err)
			continue
		}
		fmt.Printf("Agent> %s\n", reply)
		history = append(history, message{Role: "assistant", Content: reply})
	}
}

func callOpenAI(url, apiKey, model string, history []message) (string, error) {
	body, err := json.Marshal(chatRequest{
		Model:       model,
		Messages:    history,
		Temperature: 0.3,
	})
	if err != nil {
		return "", err
	}

	req, err := http.NewRequest("POST", url, bytes.NewReader(body))
	if err != nil {
		return "", err
	}
	req.Header.Set("Authorization", "Bearer "+apiKey)
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{Timeout: 60 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}

	if resp.StatusCode >= 300 {
		return "", fmt.Errorf("status %d: %s", resp.StatusCode, string(respBody))
	}

	var decoded chatResponse
	if err := json.Unmarshal(respBody, &decoded); err != nil {
		return "", err
	}
	if decoded.Error != nil {
		return "", fmt.Errorf("api error: %s", decoded.Error.Message)
	}
	if len(decoded.Choices) == 0 || decoded.Choices[0].Message.Content == "" {
		return "", fmt.Errorf("empty response")
	}
	return strings.TrimSpace(decoded.Choices[0].Message.Content), nil
}

func envOr(key, def string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return def
}

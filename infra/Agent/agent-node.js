const fs = require('fs');
const path = require('path');
const readline = require('readline');

const promptPath = path.resolve(__dirname, 'AGENTS.md');

if (!process.env.OPENAI_API_KEY) {
  console.error('Missing OPENAI_API_KEY');
  process.exit(1);
}

const systemPrompt = fs.readFileSync(promptPath, 'utf8');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

const history = [{ role: 'system', content: systemPrompt }];

const ask = () => {
  rl.question('You> ', async (input) => {
    if (!input.trim()) return ask();
    if (input.trim().toLowerCase() === ':q') {
      rl.close();
      return;
    }
    history.push({ role: 'user', content: input });
    try {
      const response = await fetch('https://api.openai.com/v1/chat/completions', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${process.env.OPENAI_API_KEY}`,
        },
        body: JSON.stringify({
          model: process.env.OPENAI_MODEL || 'gpt-4o-mini',
          messages: history,
          temperature: 0.3,
        }),
      });

      if (!response.ok) {
        const errText = await response.text();
        console.error(`API error: ${response.status} ${errText}`);
        return ask();
      }

      const data = await response.json();
      const reply = data.choices?.[0]?.message?.content?.trim() || '';
      console.log(`Agent> ${reply}`);
      history.push({ role: 'assistant', content: reply });
    } catch (err) {
      console.error('Request failed:', err);
    }
    ask();
  });
};

console.log(`Loaded system prompt from ${promptPath}`);
console.log('Type :q to exit.');
ask();

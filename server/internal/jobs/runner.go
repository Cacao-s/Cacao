package jobs

import (
	"context"
	"log/slog"
	"time"

	"github.com/Cacao/Cacao/internal/platform/config"
)

type JobFunc func(context.Context) error

type Runner struct {
	cfg  config.AppConfig
	jobs map[string]JobFunc
}

func New(cfg config.AppConfig) *Runner {
	return &Runner{cfg: cfg, jobs: make(map[string]JobFunc)}
}

func (r *Runner) Register(name string, fn JobFunc) {
	if fn == nil {
		return
	}
	r.jobs[name] = fn
}

func (r *Runner) Run(ctx context.Context) error {
	slog.Info("starting cacao jobs runner", "env", r.cfg.Env, "jobs", len(r.jobs))

	for name, job := range r.jobs {
		name := name
		job := job

		go func() {
			if err := job(ctx); err != nil {
				slog.Error("job exited", "job", name, "error", err)
			}
		}()
	}

	<-ctx.Done()

	slog.Info("cacao jobs runner stopped", "reason", ctx.Err())
	return nil
}

func Run(ctx context.Context, cfg config.AppConfig) error {
	runner := New(cfg)

	if len(runner.jobs) == 0 {
		runner.Register("noop", func(ctx context.Context) error {
			ticker := time.NewTicker(24 * time.Hour)
			defer ticker.Stop()

			for {
				select {
				case <-ctx.Done():
					return nil
				case <-ticker.C:
					slog.Info("noop job heartbeat")
				}
			}
		})
	}

	return runner.Run(ctx)
}

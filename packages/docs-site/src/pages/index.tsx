import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

export default function Home(): JSX.Element {
  return (
    <Layout
      title="OpsClaw Documentation"
      description="OpsClaw user and contributor documentation"
    >
      <header className="hero hero--opsclaw">
        <div className="container">
          <h1 className="hero__title">OpsClaw Documentation</h1>
          <p className="hero__subtitle">
            Documentation for operators and contributors building AI SRE squads with
            Slack, Discord, and Telegram workflows.
          </p>
          <p>
            <Link className="button button--secondary button--lg" to="/docs/getting-started">
              Open Getting Started
            </Link>
          </p>
        </div>
      </header>

      <main className="container margin-vert--xl">
        <section className="card-grid">
          <Link to="/docs/user-guide">
            <h3>User Guide</h3>
            <p>Install, onboard, operate, and troubleshoot OpsClaw in production.</p>
          </Link>
          <Link to="/docs/developer-guide">
            <h3>Developer Guide</h3>
            <p>Architecture, code boundaries, testing, and contributor workflows.</p>
          </Link>
          <Link to="/docs/architecture">
            <h3>Architecture</h3>
            <p>Runtime model, channels, approval safety, and delivery workflow design.</p>
          </Link>
          <Link to="/docs/blog">
            <h3>Engineering Blog</h3>
            <p>Phase-by-phase engineering recaps, tradeoffs, and reliability notes.</p>
          </Link>
        </section>
      </main>
    </Layout>
  );
}

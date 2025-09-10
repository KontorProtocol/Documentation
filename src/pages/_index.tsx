import type { ReactNode } from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          Sigil
        </Heading>
        <p className="hero__subtitle">
          An embedded domain specific language for writing secure and modular Bitcoin smart contracts.
        </p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/vision">
            Learn More
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home(): ReactNode {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title="Sigil - Smart Contracts on Kontor"
      description="Build secure and efficient blockchain smart contracts with Sigil, powered by WebAssembly and the Kontor protocol.">
      <HomepageHeader />
      <main>
        <div className="container">
          <section className="margin-vert--lg">
            <Heading as="h2">Welcome to Sigil</Heading>
            <p>
              Sigil enables blockchain smart contract development with a Rust-like experience on the Kontor protocol. The <code>sigil-example-contracts</code> zip provides five examples (<code>hello-world</code>, <code>token</code>, <code>shared-account</code>, <code>shared-account-dynamic</code>, <code>proxy</code>) to explore contract patterns, from basic queries to dynamic proxying. Each example includes Rust code, WIT interfaces, and tests. Run <code>cargo test</code> in the <code>test/</code> directory (e.g., <code>sigil-example-contracts/hello-world/test</code>) for rapid feedback on blockchain logic.
            </p>
            <p>
              The <Link to="/docs/kontor_intro">Kontor Smart Contract Guide</Link> walks through these examples, covering storage, cross-contract calls, and proxying for blockchain applications.
            </p>
            <p>
              Learn more in the <Link to="/docs/vision">Vision for Sigil</Link> to understand how Sigil and Kontor enable secure, efficient, and flexible smart contract development for the blockchain.
            </p>
          </section>
          <section className="margin-vert--lg">
            <Heading as="h2">Get Started</Heading>
            <p>
              Check the <Link to="/docs/getting-started">Getting Started</Link> guide to set up your environment and explore <code>sigil-example-contracts</code>. Future testing layers will include live Kontor instances and client-side SDK integration with Bitcoin testnet for full blockchain validation.
            </p>
          </section>
        </div>
      </main>
    </Layout>
  );
}

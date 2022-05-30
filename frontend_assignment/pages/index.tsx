import detectEthereumProvider from '@metamask/detect-provider';
import { Strategy, ZkIdentity } from '@zk-kit/identity';
import { generateMerkleProof, Semaphore } from '@zk-kit/protocols';
import { providers } from 'ethers';
import Head from 'next/head';
import React from 'react';
import styles from '../styles/Home.module.css';
import MyForm from 'components/Form';
export default function Home() {
  const [logs, setLogs] = React.useState('Connect your wallet and greet!');

  async function greet(str: string) {
    setLogs('Creating your Semaphore identity...');

    const provider = (await detectEthereumProvider()) as any;

    await provider.request({ method: 'eth_requestAccounts' });

    const ethersProvider = new providers.Web3Provider(provider);
    const signer = ethersProvider.getSigner();
    const message = await signer.signMessage(
      str ?? 'Sign this message to create your identity!'
    );

    const identity = new ZkIdentity(Strategy.MESSAGE, message);
    const identityCommitment = identity.genIdentityCommitment();
    const arr = await (await fetch('./identityCommitments.json')).json();
    const identityCommitments = [...arr, identityCommitment];
    const merkleProof = generateMerkleProof(
      20,
      BigInt(0),
      identityCommitments,
      identityCommitment
    );

    setLogs('Creating your Semaphore proof...');

    const greeting = str ?? 'Hello world';

    const witness = Semaphore.genWitness(
      identity.getTrapdoor(),
      identity.getNullifier(),
      merkleProof,
      merkleProof.root,
      greeting
    );

    const { proof, publicSignals } = await Semaphore.genProof(
      witness,
      './semaphore.wasm',
      './semaphore_final.zkey'
    );
    const solidityProof = Semaphore.packToSolidityProof(proof);

    const response = await fetch('/api/greet', {
      method: 'POST',
      body: JSON.stringify({
        greeting,
        nullifierHash: publicSignals.nullifierHash,
        solidityProof: solidityProof,
      }),
    });

    if (response.status === 500) {
      const errorMessage = await response.text();

      setLogs(errorMessage);
    } else {
      const value = await response.text();
      setLogs(
        `Your anonymous greeting is onchain :) Your greet text is ${greeting} and in bytes it is ${value}`
      );
    }
  }

  return (
    <div className={styles.container}>
      <Head>
        <title>Greetings</title>
        <meta
          name='description'
          content='A simple Next.js/Hardhat privacy application with Semaphore.'
        />
        <link rel='icon' href='/favicon.ico' />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>Greetings</h1>

        <p className={styles.description}>
          A simple Next.js/Hardhat privacy application with Semaphore.
        </p>

        <div className={styles.logs}>{logs}</div>
        <MyForm greet={greet} />
        {/* <div onClick={() => greet()} className={styles.button}>
          Greet
        </div> */}
      </main>
    </div>
  );
}

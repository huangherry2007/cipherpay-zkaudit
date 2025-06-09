import type { NextApiRequest, NextApiResponse } from 'next';
import { spawn } from 'child_process';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  if (req.method !== 'POST') {
    res.status(405).json({ error: 'Method not allowed' });
    return;
  }

  const { proof, publicInputs } = req.body;
  if (!proof || !publicInputs) {
    res.status(400).json({ error: 'Missing proof or publicInputs' });
    return;
  }

  // Prepare input for Rust verifier
  const input = JSON.stringify({ proof, publicInputs });

  // Call the Rust verifier CLI
  const child = spawn('cargo', ['run', '--manifest-path', './rust/Cargo.toml'], {
    cwd: process.cwd() + '/rust',
    stdio: ['pipe', 'pipe', 'pipe'],
  });

  let output = '';
  child.stdout.on('data', (data) => {
    output += data.toString();
  });

  let error = '';
  child.stderr.on('data', (data) => {
    error += data.toString();
  });

  child.stdin.write(input);
  child.stdin.end();

  child.on('close', (code) => {
    if (code === 0) {
      try {
        const result = JSON.parse(output);
        res.status(200).json(result);
      } catch (e) {
        res.status(500).json({ error: 'Failed to parse verifier output', output });
      }
    } else {
      res.status(500).json({ error: 'Verifier failed', details: error });
    }
  });
}

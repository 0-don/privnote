import apaterNetlify from '@sveltejs/adapter-netlify';
import adapaterNode from '@sveltejs/adapter-node';
import adapterVercel from '@sveltejs/adapter-vercel';
import { vitePreprocess } from '@sveltejs/kit/vite';
import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';

const file = fileURLToPath(new URL('package.json', import.meta.url));
const json = readFileSync(file, 'utf8');
const pkg = JSON.parse(json);

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [vitePreprocess({})],
  kit: {
    version: {
      name: pkg.version
    },
    adapter: process.env.VERCEL_ENV ? adapterVercel() : process.env.NETLIFY ? apaterNetlify() : adapaterNode(),
    csrf: {
      checkOrigin: false
    }
  }
};

export default config;

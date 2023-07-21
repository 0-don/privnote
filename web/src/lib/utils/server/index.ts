import { env } from '$env/dynamic/private';
import type { Captcha, Messages, ResponseBody } from '$lib/@types';
import type { RequestEvent } from '@sveltejs/kit';
import type { Dispatcher } from 'undici';
import { request } from 'undici';
import { COOKIE, COOKIE_SERIALIZE_OPTIONS } from './constants';

export const client = async <T = Messages[]>(
  path: string,
  options?: Parameters<typeof request>['1']
): Promise<{ body: { json: () => Promise<T> } } & Dispatcher.ResponseData> => {
  return await request(env.SECRET_ENDPOINT + path, {
    headers: { 'Content-Type': 'application/json', SECRET: env.SECRET_API_SECRET, ...options?.headers },
    ...options
  });
};

export const getCaptcha = async ({ cookies }: RequestEvent): Promise<ResponseBody<Partial<Captcha>>> => {
  try {
    const { text, tag } = await (
      await client<Captcha>('auth/captcha', {
        method: 'GET'
      })
    ).body.json();

    cookies.set(COOKIE, text, COOKIE_SERIALIZE_OPTIONS);

    return { data: { tag } };
  } catch (err) {
    return { messages: [{ message: JSON.stringify(err), path: 'error' }] };
  }
};

import { dev } from '$app/environment';
import { env } from '$env/dynamic/private';
import type { Messages, ResponseBody } from '$lib/@types';

export const COOKIE = 'captcha';

export const TOKEN_EXPIRES_IN = 60 * 60; // 1 Minute

export const COOKIE_SERIALIZE_OPTIONS: import('cookie').CookieSerializeOptions = {
  httpOnly: true,
  path: '/',
  maxAge: TOKEN_EXPIRES_IN,
  sameSite: !dev ? 'lax' : 'none',
  secure: true, //process.env.NODE_ENV === 'production' ? true : false, // cookie only works in https
  domain: env.SECRET_CORS_ORIGIN
};

import { dev } from '$app/environment';
import { env } from '$env/dynamic/private';

export const COOKIE = 'csrf';

export const TOKEN_EXPIRES_IN = 60 * 60; // 1 Minute

export const COOKIE_SERIALIZE_OPTIONS: import('cookie').CookieSerializeOptions = {
  httpOnly: true,
  path: '/',
  maxAge: TOKEN_EXPIRES_IN,
  sameSite: !dev ? 'lax' : 'none',
  secure: true //process.env.NODE_ENV === 'production' ? true : false, // cookie only works in https
};

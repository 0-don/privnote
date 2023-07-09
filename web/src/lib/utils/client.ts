import { request } from 'undici';
import { env } from '$env/dynamic/private';

export const client = async <T>(path: string, options?: Parameters<typeof request>['1']): ReturnType<typeof request> => {
  return await request(env.SECRET_ENDPOINT + path, { headers: { 'Content-Type': 'application/json' }, ...options }) ;
};

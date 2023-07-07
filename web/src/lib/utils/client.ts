import { request } from 'undici';
import { env } from '$env/dynamic/private';

export const client = async (path: string, options?: Parameters<typeof request>['1']): ReturnType<typeof request> => {
  console.log('client', env.SECRET_ENDPOINT);
  return await request(env.SECRET_ENDPOINT + path, options);
};

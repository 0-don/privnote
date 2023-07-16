import { dev } from '$app/environment';
import type { ResponseBody, Messages } from '$lib/@types';

export const debugLog = (data: ResponseBody, form: ResponseBody) =>
  dev
    ? {
        data: [
          ...(data?.data ? [{ key: 'response', path: 'data', message: data?.data }] : []),
          ...(form?.data ? [{ key: 'request', path: 'form', message: form?.data }] : [])
        ] as Messages[],
        form: (form?.messages || []).map((m) => ({ ...m, key: 'form' })),
        captcha: (data?.messages || []).map((m) => ({ ...m, key: 'captcha' }))
      }
    : {};

import type { Messages, ResponseBody } from '$lib/@types';
import { NoteSchema } from '$lib/schemas/note.schema';
import type { PageServerLoad } from './$types';
import type { Actions } from '@sveltejs/kit';
import { z } from 'zod';
import { COOKIE } from '$lib/utils/server/constants';
import { client, getCaptcha } from '$lib/utils/server';

export const load = (async (options): Promise<ResponseBody> => await getCaptcha(options)) satisfies PageServerLoad;

export const actions = {
  default: async ({ request, cookies }): Promise<ResponseBody> => {
    const form = Object.fromEntries(await request.formData());
    try {
      const text = cookies.get(COOKIE);

      const data = NoteSchema.parse(form);

      const response = await (
        await client<ResponseBody>('note', {
          method: 'POST',
          body: JSON.stringify({ ...data, text })
        })
      ).body.json();

      return response;
    } catch (err) {
      if (err instanceof z.ZodError) {
        return {
          messages: [
            ...(err.issues.map(({ message, path }) => ({
              message: message,
              path: path.join('.'),
              value: form?.[path.join('.')] || ''
            })) as Messages[]),
            ...Object.keys(form).map((k) => ({ path: k, value: form?.[k] || '' }) as Messages)
          ]
        };
      } else {
        return {
          messages: [
            {
              message: JSON.stringify(err),
              path: 'error',
              ...Object.keys(form).map((k) => ({ path: k, value: form?.[k] || '' }) as Messages)
            }
          ]
        };
      }
    }
  }
} satisfies Actions;

import type { Captcha, Messages, ResponseBody } from '$lib/@types';
import { client } from '$lib/utils';
import { NoteSchema } from '$lib/schemas/note.schema';
import type { PageServerLoad } from './$types';
import type { Actions } from '@sveltejs/kit';
import { z } from 'zod';
import { error } from 'console';
import { COOKIE, COOKIE_SERIALIZE_OPTIONS } from '$lib/utils/server/constants';

export const load = (async ({ cookies }): Promise<ResponseBody> => {
  try {
    const { text, tag } = await (
      await client<Captcha>('auth/captcha', {
        method: 'GET'
      })
    ).body.json();

    cookies.set(COOKIE, text, COOKIE_SERIALIZE_OPTIONS);

    return { data: { tag } };
  } catch (error) {
    return { messages: [{ message: 'Server error', path: 'error' }] };
  }
}) satisfies PageServerLoad;

export const actions = {
  default: async ({ request, cookies }): Promise<ResponseBody> => {
    try {
      const text = cookies.get(COOKIE);

      const form = Object.fromEntries(await request.formData());

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
          messages: err.issues.map(({ message, path }) => ({
            message: message,
            path: path.join('.')
          })) as Messages[]
        };
      } else {
        error(err);
        return { messages: [{ message: 'Unknown error', path: 'error' }] };
      }
    }
  }
} satisfies Actions;

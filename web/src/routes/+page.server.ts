/* eslint-disable @typescript-eslint/ban-ts-comment */
import type { Captcha, JSONValue, Messages, ResponseBody } from '$lib/@types';
import { client } from '$lib/utils/client';
import { COOKIE, COOKIE_SERIALIZE_OPTIONS } from '$lib/utils/constants';
import { NoteSchema } from '$lib/utils/schemas/note.schema';
import type { PageServerLoad } from './$types';
import type { Actions } from '@sveltejs/kit';
import { z } from 'zod';
// @ts-ignore
import { error } from 'console';

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

      console.log(form);

      const data = NoteSchema.parse(form);

      const messages = await (
        await client<string | Messages[]>('note', {
          method: 'POST',
          body: JSON.stringify({ ...data, text })
        })
      ).body.json();

      return Array.isArray(messages) ? { messages } : { data: messages };
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

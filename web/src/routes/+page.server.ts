/* eslint-disable @typescript-eslint/ban-ts-comment */
import type { Notification } from '$lib/@types';
import { client } from '$lib/utils/client';
import { COOKIE, COOKIE_SERIALIZE_OPTIONS } from '$lib/utils/constants';
import { NoteSchema } from '$lib/utils/schemas/note.schema';
import type { PageServerLoad } from './$types';
import type { Actions } from '@sveltejs/kit';
import { z } from 'zod';
// @ts-ignore
import { error } from 'console';

export const load = (async ({ cookies }) => {
  try {
    const { text, id } = await (
      await client<{ text: string; id: number }>('auth/captcha', {
        method: 'GET'
      })
    ).body.json();

    cookies.set(COOKIE, text, COOKIE_SERIALIZE_OPTIONS);

    return { id };
  } catch (error) {
    return [{ message: 'Server error', path: 'error' }];
  }
}) satisfies PageServerLoad;

export const actions = {
  default: async ({ request, cookies }): Promise<Notification[]> => {
    try {
      const text = cookies.get(COOKIE);

      const form = Object.fromEntries(await request.formData());

      const data = NoteSchema.parse(form);

      const message = await (
        await client<string>('note', {
          method: 'POST',
          body: JSON.stringify({ ...data, text })
        })
      ).body.json();

      return [{ message, path: 'ok' }];
    } catch (err) {
      if (err instanceof z.ZodError) {
        return err.issues.map(({ message, path }) => ({
          message: message,
          path: path.join('.')
        })) as Notification[];
      } else {
        error(err);
        return [{ message: 'Unknown error', path: 'error' }];
      }
    }
  }
} satisfies Actions;

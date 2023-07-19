import type { Note, ResponseBody } from '$lib/@types';
import type { Actions, Redirect } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { client, getCaptcha } from '$lib/utils/server';
import { deepMerge } from '$lib/utils';
import { COOKIE } from '$lib/utils/server/constants';

export const load = (async (options): Promise<ResponseBody | Redirect> => {
  if (!options.params?.id) return redirect(302, '/');

  try {
    const body = await (
      await client<ResponseBody<{ note: Note; alert: string }>>(`note/${options.params.id}`, {
        method: 'GET'
      })
    ).body.json();

    if (!body.data?.note.id) return redirect(302, '/');

    const captcha = await getCaptcha(options);

    return deepMerge(body, captcha);
  } catch (error) {
    return { messages: [{ message: 'Server error', path: 'error' }] };
  }
}) satisfies PageServerLoad;

export const actions = {
  default: async ({ request, cookies }) => {
    try {
      const text = cookies.get(COOKIE);

      const form = Object.fromEntries(await request.formData());

      console.log(text, form);
    } catch (err) {}
  }
} satisfies Actions;

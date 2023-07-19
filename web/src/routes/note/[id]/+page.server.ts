import type { Note, ResponseBody } from '$lib/@types';
import type { Actions, Redirect } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { client, getCaptcha } from '$lib/utils/server';

export const load = (async (options): Promise<ResponseBody | Redirect> => {
  if (!options.params?.id) return redirect(302, '/');

  try {
    const body = await (
      await client<ResponseBody<{ data: Note; alert: string }>>(`note/${options.params.id}`, {
        method: 'GET'
      })
    ).body.json();

    if (!body.data?.data.id) return redirect(302, '/');

    const captcha = await getCaptcha(options);

    return deepMerge(body, captcha);
  } catch (error) {
    return { messages: [{ message: 'Server error', path: 'error' }] };
  }
}) satisfies PageServerLoad;

export const actions = {
  default: async ({ request, cookies }) => {
    try {
      const form = Object.fromEntries(await request.formData());

      console.log(form);
    } catch (err) {}
  }
} satisfies Actions;

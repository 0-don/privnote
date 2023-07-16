import type { ResponseBody, Note } from '$lib/@types';
import { client } from '$lib/utils/client';
import type { Redirect } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = (async ({ params }): Promise<ResponseBody | Redirect> => {
  if (!params?.id) return redirect(302, '/');

  try {
    const note = await (
      await client<ResponseBody<{ data: Note; alert: string }>>(`note/${params.id}`, {
        method: 'GET'
      })
    ).body.json();

    return note;
  } catch (error) {
    return { messages: [{ message: 'Server error', path: 'error' }] };
  }
}) satisfies PageServerLoad;

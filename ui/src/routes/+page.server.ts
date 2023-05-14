import type { PageServerLoad } from './$types';
import showdown from 'showdown';
import type { Entry } from '$lib/types/entry';

export const load: PageServerLoad<{
	entries: Entry[];
}> = async ({}) => {
	const converter = new showdown.Converter();
	let res = await fetch(`http://127.0.0.1:8080/v1/changelog`);
	const data: Omit<Entry, 'html'>[] = await res.json();
	return {
		entries: data.map((e) => ({
			...e,
			html: converter.makeHtml(e.text),
			created_at: new Date(e.created_at),
			updated_at: new Date(e.updated_at)
		}))
	};
};

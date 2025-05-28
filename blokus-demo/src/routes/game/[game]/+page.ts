import type { PageLoad } from './$types.ts';
import * as base64 from '$lib/base64.ts';
import { Blokus as BlokusBitmap  } from "$lib/cgtjs.ts";

export const load: PageLoad = ({ params, url }) => {
    const polyominosBase64Serialized = url.searchParams.get('polyominos')?.split(',') ?? [];
    const polyominos = polyominosBase64Serialized.map(board => BlokusBitmap.deserialize(base64.decode(board)));
    const board = BlokusBitmap.deserialize(base64.decode(params.game));
    const children: BlokusBitmap[] = [];
    for (const child of board.moves(polyominos)) {
        let exists = false;
        for (const existingChild of children) {
            if (existingChild.isEqualTo(child)) {
                exists = true;
                break;
            }
        }
        if (!exists) {
            children.push(child);
        }
    }
	return { board, children, polyominos };
};

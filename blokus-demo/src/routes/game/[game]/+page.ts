import type { PageLoad } from './$types.ts';
import { Blokus as BlokusBitmap  } from "$lib/cgtjs.ts";

export const load: PageLoad = ({ params }) => {
    const polyominos = [
		BlokusBitmap.fromString(`
			csc
			sis
			sis
			csc`)
	];
    const board = BlokusBitmap.deserializeAscii(params.game);
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
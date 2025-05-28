import { Blokus as BlokusBitmap } from '$lib/cgtjs.ts';
import * as base64 from '$lib/base64.ts';

export function load({ url }) {
    const polyominosBase64Serialized = url.searchParams.get('polyominos')?.split(',')?.filter(s => s.length > 0) ?? [];
    const polyominos = polyominosBase64Serialized.map(board => BlokusBitmap.deserialize(base64.decode(board)));
    return { polyominos, url };
}
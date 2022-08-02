import * as utils from '../utils';

const assert = (value) => {
    if (!value) throw new Error( "Invalid model");
}

export const utilisation = (proportion) => {
    return utils.bn(proportion * 1_000_000_000);
}

export const interest = (value) => {
    return utils.bn(value * 1_000_000_000);
}

export const slope = (slope) => {
    return utils.bn(slope * 1_000_000_000);
}

export const convert = (x, y, s) => {
    return [utilisation(x), interest(y), slope(s)]
    // return [x, y, s]
}

export const point = (x, y) => {
    return { x, y }
}

export const from = (points) => {
    assert(points[0].x == 0);
    assert(points[points.length-1].x == 1); 

    const slopes = []
    for (let i = 0; i < points.length - 1; i++) {
        let current = points[i];
        let next = points[i+1];

        assert(current.x < next.x);
        assert(current.y < next.y);

        let s = (next.y - current.y) / (next.x - current.x);
        slopes.push(convert(current.x, current.y, s))

    }
    // console.log(slopes);
    return slopes;
}
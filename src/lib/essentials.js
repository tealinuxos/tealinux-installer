import prettyBytes from 'pretty-bytes';

export const prettySize = (size) => {
    return prettyBytes(size * 512, { binary: true });
};

# audio_painter

This program takes two audio files as input; a "target" and "paint". The target is split into small pieces, and the paint is searched for the closest matches to those pieces.

## Usage
```bash
$ audio_painter -t <target> -p <paint>
```

### Additional options
- `-o` specify the path for the resulting .wav file (defaults to `./out.wav`)
- `-c` specify the size of the chunks that the target audio should be split into (defaults to 500 samples)
- `-j` specify the number of samples the search head should jump by on each iteration (defaults to 200)
- `-m` specify the mix between dry and wet audio when rendering the output (defaults to 0)
- `-n` normalize the target and paint audio

## Limitations
Dynamic chunk duration currently not implemented, meaning the chunks are all of uniform length. This means that some interesting audio elements will be truncated, potentially causing clicking and other audio artefacts.

There's currently only support for 16 bit wav files. I'll add support for more formats in due course.

## Things that are coming
1. Dynamic chunk duration
2. Support for more wave formats
3. Gradient descent searching

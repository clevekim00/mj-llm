# N0 synthetic fixtures

`red.png` is a project-created 64×64 RGB PNG with every pixel `(255, 0, 0)`.
No user photo or third-party image is included. It tests image decoding and
finite normalized embedding output, not semantic image-retrieval quality.

The fixed Korean query and positive/negative text are defined in
`crates/n0-probe/src/lib.rs`. The repeat cosine floor is 0.9999; normalized
vector tolerance is 0.001. These smoke checks do not replace G4/G5 corpora.

The generation fixture asks for `2 + 2` with thinking disabled, 32 output tokens,
512 context tokens, and top-p sampler, k=1 / p=1 / temperature=1 / seed=0. It checks nonempty
structured SDK output, not general answer quality or streaming/cancellation.

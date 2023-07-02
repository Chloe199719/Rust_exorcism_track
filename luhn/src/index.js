function stuff(s) {
  const d =
    [...s]
      .filter((e) => e !== " ")
      .map((e) => parseInt(e))
      .map((e, i) => (i % 2 === 0 ? e * 2 : e))
      .map((e) => (e > 9 ? e - 9 : e))
      .reduce((a, b) => a + b, 0) %
      10 ===
    0;
  console.log(d);
}

stuff("095 245 88");

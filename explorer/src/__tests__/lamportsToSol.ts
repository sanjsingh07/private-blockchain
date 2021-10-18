import { expect } from "chai";
import { caratsToGema, CARATS_PER_GEMA } from "utils";
import BN from "bn.js";

describe("caratsToGema", () => {
  it("0 carats", () => {
    expect(caratsToGema(new BN(0))).to.eq(0.0);
  });

  it("1 carat", () => {
    expect(caratsToGema(new BN(1))).to.eq(0.000000001);
    expect(caratsToGema(new BN(-1))).to.eq(-0.000000001);
  });

  it("1 GEMA", () => {
    expect(caratsToGema(new BN(CARATS_PER_GEMA))).to.eq(1.0);
    expect(caratsToGema(new BN(-CARATS_PER_GEMA))).to.eq(-1.0);
  });

  it("u64::MAX carats", () => {
    expect(caratsToGema(new BN(2).pow(new BN(64)))).to.eq(
      18446744073.709551615
    );
    expect(caratsToGema(new BN(2).pow(new BN(64)).neg())).to.eq(
      -18446744073.709551615
    );
  });
});

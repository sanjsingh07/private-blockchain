import React from "react";
import { BigNumber } from "bignumber.js";
import { GemaBalance } from "utils";

export function BalanceDelta({
  delta,
  isGema = false,
}: {
  delta: BigNumber;
  isGema?: boolean;
}) {
  let sols;

  if (isGema) {
    sols = <GemaBalance carats={delta.toNumber()} />;
  }

  if (delta.gt(0)) {
    return (
      <span className="badge badge-soft-success">
        +{isGema ? sols : delta.toString()}
      </span>
    );
  } else if (delta.lt(0)) {
    return (
      <span className="badge badge-soft-warning">
        {isGema ? <>-{sols}</> : delta.toString()}
      </span>
    );
  }

  return <span className="badge badge-soft-secondary">0</span>;
}

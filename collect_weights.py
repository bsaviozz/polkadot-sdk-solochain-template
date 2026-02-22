#!/usr/bin/env python3
import argparse
import csv
import re
import subprocess
from pathlib import Path

MEDIAN_RE = re.compile(
    r"Median Slopes Analysis.*?Time\s*~=\s*(\d+)\s*µs", re.S
)
MINSQ_RE = re.compile(
    r"Min Squares Analysis.*?Time\s*~=\s*(\d+)\s*µs", re.S
)

def run_and_parse(cmd: list[str]) -> tuple[int, int]:
    p = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    out = p.stdout
    if p.returncode != 0:
        raise RuntimeError(out)

    m1 = MEDIAN_RE.search(out)
    m2 = MINSQ_RE.search(out)
    if not m1 or not m2:
        raise RuntimeError("Could not parse both times from output:\n" + out)

    return int(m1.group(1)), int(m2.group(1))

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--node", default="./target/release/solochain-template-node")
    ap.add_argument("--pallet", default="pallet_template")
    ap.add_argument("--chain", default="dev")
    ap.add_argument("--steps", type=int, default=50)
    ap.add_argument("--repeat", type=int, default=20)
    ap.add_argument("--runs", type=int, default=30)
    ap.add_argument("--csv", default="./bench_out/time_verification.csv")
    args = ap.parse_args()

    node = Path(args.node)
    if not node.exists():
        raise SystemExit(f"Node binary not found: {node}")

    csv_path = Path(args.csv)
    csv_path.parent.mkdir(parents=True, exist_ok=True)

    algos = [
        ("dilithium", "verify_dilithium"),
        ("sr25519", "verify_sr25519"),
        ("ecdsa", "verify_ecdsa"),
    ]

    write_header = not csv_path.exists()
    with csv_path.open("a", newline="") as f:
        w = csv.writer(f)
        if write_header:
            w.writerow(["algorithm", "iteration", "time_us_median_slopes", "time_us_min_squares"])

        for algo, extrinsic in algos:
            for i in range(1, args.runs + 1):
                cmd = [
                    str(node), "benchmark", "pallet",
                    "--chain", args.chain,
                    "--pallet", args.pallet,
                    "--extrinsic", extrinsic,
                    "--steps", str(args.steps),
                    "--repeat", str(args.repeat),
                ]
                t_med, t_min = run_and_parse(cmd)
                w.writerow([algo, i, t_med, t_min])
                print(f"{algo} run {i}: median={t_med} µs, minsq={t_min} µs")

    print(f"Wrote: {csv_path}")

if __name__ == "__main__":
    main()

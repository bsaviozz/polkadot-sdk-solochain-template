#!/usr/bin/env python3
import argparse, csv, re, subprocess
from pathlib import Path

# We keep only ONE value: Median Slopes "Time ~= X µs"
TIME_RE = re.compile(r"Median Slopes Analysis.*?Time\s*~=\s*(\d+)\s*µs", re.S)

def run_and_parse(cmd) -> int:
    p = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    out = p.stdout
    if p.returncode != 0:
        raise RuntimeError(out)

    m = TIME_RE.search(out)
    if not m:
        raise RuntimeError("Could not parse Median Slopes time from output:\n" + out)

    return int(m.group(1))

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--node", default="./target/release/solochain-template-node")
    ap.add_argument("--pallet", default="pallet_template")
    ap.add_argument("--chain", default="dev")
    ap.add_argument("--steps", type=int, default=1)
    ap.add_argument("--repeat", type=int, default=1)
    ap.add_argument("--runs", type=int, default=500)
    ap.add_argument("--csv", default="./bench_out/verify_time_samples.csv")
    ap.add_argument("--extrinsic", default="verify_dilithium")
    args = ap.parse_args()

    node = Path(args.node)
    if not node.exists():
        raise SystemExit(f"Node binary not found: {node}")

    csv_path = Path(args.csv)
    csv_path.parent.mkdir(parents=True, exist_ok=True)

    write_header = not csv_path.exists()
    with csv_path.open("a", newline="") as f:
        w = csv.writer(f)
        if write_header:
            w.writerow(["algorithm", "iteration", "verification_time_us", "steps", "repeat"])

        for i in range(1, args.runs + 1):
            cmd = [
                str(node), "benchmark", "pallet",
                "--chain", args.chain,
                "--pallet", args.pallet,
                "--extrinsic", args.extrinsic,
                "--steps", str(args.steps),
                "--repeat", str(args.repeat),
            ]
            t_us = run_and_parse(cmd)
            w.writerow([args.extrinsic, i, t_us, args.steps, args.repeat])
            print(f"run {i}: time={t_us} µs")

    print(f"Wrote: {csv_path}")

if __name__ == "__main__":
    main()
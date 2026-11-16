"""Build controlled A4 cross-ring transit comparisons from the full practice ring.

These are hypothetical route/contract tests, not copies of the hidden live worlds.
"""

from pathlib import Path
import re

HERE = Path(__file__).parent
BASE = (HERE / "full.toml").read_text()
PEER = "../../examples/settlements/peering/target/wasm32-unknown-unknown/release/settle_peering.wasm"
TRANSIT = "../../examples/settlements/transit/target/wasm32-unknown-unknown/release/settle_transit.wasm"


def agreement(kind, a, b, price=0):
    program = PEER if kind == "peer" else TRANSIT
    terms = "{ }" if kind == "peer" else f"{{ price = {price}, min_on_time = 0.0, refund = 0.0 }}"
    return (
        "\n[[a4.agreement]]\n"
        f'kind = "{kind}"\na = {a}\nb = {b}\n'
        f'program = "{program}"\nterms = {terms}\nmax = 400\n'
    )


def hardware(owner, near_hub, far_ixp, mbps):
    remote = owner * 10 + 5
    block = f"\n[[switch]]\nid = {remote}\nas_id = {owner}\n"
    block += f"\n[[link]]\na = {near_hub}\nb = {remote}\ncapacity_mbps = {mbps}\nlatency_us = {12000 if mbps == 5 else 22000}\n"
    sectors = [far_ixp, (far_ixp - 1) % 8]
    for sector in sectors:
        for target in range(4 * sector + 1, 4 * sector + 5):
            hub = target * 10 + (1 if sector == far_ixp else 2)
            block += f"\n[[link]]\na = {remote}\nb = {hub}\ncapacity_mbps = 20\nlatency_us = 1000\n"
    return block


def base_for_us(our_as):
    pattern = rf'(\[\[a4\.as\]\]\nid = {our_as}\nsector = \d+\nslot = \d+\nprogram = ")[^"]+("\n)'
    text, replacements = re.subn(pattern, r'\1../../switch.wasm\2', BASE)
    assert replacements == 1
    return text


# Owner and endpoints are read from the user's Nov 16 Worlds maps.
# W2: 123455 (AS19) crosses IXP4 -> IXP0; W3: Lucky Comets (AS18)
# crosses IXP4 -> IXP0. Each owner peers with far AS2. The buyer is
# adjacent to us but not directly adjacent to the cross-ring owner.
PILOTS = {
    "w2": dict(our=23, owner=19, near_hub=191, far_ixp=0, far_as=2, buyer=27, mbps=10),
    "w3": dict(our=15, owner=18, near_hub=181, far_ixp=0, far_as=2, buyer=11, mbps=5),
    "w4": dict(our=23, owner=6, near_hub=61, far_ixp=5, far_as=17, buyer=27, mbps=5),
}

for name, p in PILOTS.items():
    base = base_for_us(p["our"]) + hardware(p["owner"], p["near_hub"], p["far_ixp"], p["mbps"])
    base += agreement("peer", p["owner"], p["far_as"])
    if name == "w4":
        # W4's owner lands at our own IXP5, where we can peer directly
        # with the same far-side AS used for this test.
        base += agreement("peer", p["our"], p["far_as"])
    for variant in ("no_deal", "peer_owner", "buy_transit", "buy_and_sell", "peer_and_sell", "buy_and_sell_no_far", "peer_owner_peer_buyer", "buy_transit_peer_buyer", "peer_and_sell_low", "buy_and_sell_low"):
        content = base
        if variant == "buy_and_sell_no_far":
            content = base_for_us(p["our"]) + hardware(p["owner"], p["near_hub"], p["far_ixp"], p["mbps"])
            if name == "w4":
                content += agreement("peer", p["our"], p["far_as"])
        if variant in ("peer_owner", "peer_and_sell", "peer_owner_peer_buyer", "peer_and_sell_low"):
            content += agreement("peer", p["our"], p["owner"])
        if variant in ("buy_transit", "buy_and_sell", "buy_and_sell_no_far", "buy_transit_peer_buyer", "buy_and_sell_low"):
            content += agreement("transit", p["our"], p["owner"], 0.5)
        if variant in ("buy_and_sell", "peer_and_sell", "buy_and_sell_no_far"):
            content += agreement("transit", p["buyer"], p["our"], 1.0)
        if variant in ("peer_owner_peer_buyer", "buy_transit_peer_buyer"):
            content += agreement("peer", p["buyer"], p["our"])
        if variant in ("peer_and_sell_low", "buy_and_sell_low"):
            content += agreement("transit", p["buyer"], p["our"], 0.3)
        (HERE / f"cross-ring-{name}-{variant}.toml").write_text(content)

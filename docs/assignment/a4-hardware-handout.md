# A4 hardware and auctions handout

Source: `a4-hardware-handout-2026-11-07.pdf`. Text extracted from all five pages for search and standing context. Consult the PDF for tables and the auction figure; the PDF is authoritative for layout. Values are provisional until game opening; use root `rules.json` for keyed numbers.

## Source PDF page 1

Assignment 4: Hardware and auctions
CS 356 UT Austin · Fall 2026 · Student handout
Every team starts with the same network. Hardware is how you make yours different: more capacity
where your traffic is, a short way to the far side of the ring, your own link to a big neighbour, or links
that come back sooner after a failure. There is no shop. Everything is sold in three auctions, for
credits, and nobody knows the right prices in advance, so you set them by bidding. This handout
covers the five kinds of hardware, how an auction works, what happens after you buy, and Auction
1’s lots. It adds to the A4 rule book (the last section of the A4 handout, updated Sat Nov 7), which
still holds. Give this handout to your AI agent with the rule book and rules.json. Every number
is a key in rules.json, which may change until the game opens.
1 Credits
Credits (cr) are separate in each world: 100cr when the game opens (credits.at_open) and 100cr
every Monday after that (credits.per_monday), carried over up to 200cr (credits.carry_max);
credits held in bids (Section 3) do not count toward that cap. They buy hardware and nothing
else, they are not dollars, and they are not part of your score. Hardware bought in a world stays
in that world.
2 The five kinds of hardware
Hardware adds to the network every AS starts with (Table 1). Each item is one unit: you buy it
in one world and install it on your own AS there. This section says what each does, and Table 2,
at its end, sums them up. The numbers are today’s: the current values are in rules.json, under
the keys in the tables, and may change until the game opens. All rates are real rates (the practice
worlds run at a tenth of them).
Link Capacity Latency rules.json
Crossing: hub to hub, through your pass 100Mb/s 10ms network.crossing
IXP port: to a neighbour, at each IXP 200Mb/s 1ms network.ixp_port
Tier-1 link, at each hub 40Mb/s 32ms network.tier1_link
Hub to access switch 1000Mb/s 6ms network.hub_to_access
Buffer: 12,500 bytes on every port of every switch buffer_bytes_per_port
Program memory: 128MiB for your switch program switch_program.memory_bytes
Table 1: The network every AS starts with.
Second cable. A second cable through your pass, beside your crossing, so your crossing
carries 200Mb/s instead of 100. It is in the same pass, so it fails whenever your crossing fails:
more capacity, same risk.
Link across the ring. A link from your hub in one of your towns to a new switch of yours
at the IXP directly across the ring (X to X , the dotted lines on the map). The new
i i+4
switch has a 200Mb/s port at the far IXP, so you can make agreements with the ASes there,
just as at your own two IXPs. The ordinary kind carries 100Mb/s in 22ms; the microwave
kind half as much, 10ms faster. Without one, the far side is several crossings and IXPs away,

## Source PDF page 2

or two Tier-1 links (64ms); Trading’s target is 30ms and Calls’ 80ms (classes). The link
carries nothing until you have an agreement with an AS at the far IXP: its value is the deals
it makes possible. It never fails in the failure schedule, but your link to an AS at the far IXP
fails when that AS’s port there fails, as at your own IXPs.
Private interconnect. Your own link to one neighbour at one of your IXPs, beside the IXP
port: 200Mb/s more between your hub and theirs in that town, for traffic between the two
of you only. The neighbour must agree, on the website, before it is installed. Traffic flows
over it only while you have an active agreement; the link stays when an agreement ends, so
a new agreement uses it again. It fails when that IXP port fails.
Upgrades, four kinds, each a separate lot:
– link capacity: 100Mb/s more on one hub-to-access link of yours, or on a link you bought.
Not on your crossing (that is the second cable) or a Tier-1 link (its capacity cannot be
bought);
– buffer: 12,500 bytes more on every port of one of your switches, which doubles its buffer;
– faster port: 200Mb/s more on one of your IXP ports, or on the far port of a link across
the ring;
– program memory: 128MiB more (256MiB in all) for your switch program on your
switches in this world, not for its routing-policy calls. You submit one program for all
four worlds, and it must still pass its dry run at 128MiB, so the upgrade is headroom,
not a new floor.
Repair crew. After a failure in this world, each of your links that failed comes back in half
the time it would otherwise have taken: your crossing and second cable in a pass failure, your
IXP ports and any private interconnect on them. A pass failure is shorter, not gone, so a
backup agreement through the other pass can still pay.
Traffic over a private interconnect, or over the far port of a link across the ring, is metered and
settled like traffic at an IXP. Two units in the same place add up (two second cables add 200Mb/s);
a second repair crew adds nothing. A lot limits how many units one team may win in it (Section 5);
what you own is not limited.
Item What it adds Fails key in rules.json: hardware....
Second cable +100Mb/s on your crossing with your crossing second_cable
Link across the ring, 100Mb/s, 22ms, to your never cross_ring.ordinary,
ordinary switch at the far IXP cross_ring.far_port_mbps
(200Mb/s port)
Link across the ring, the same, at 50Mb/s and never cross_ring.microwave
microwave 12ms
Private interconnect +200Mb/s to one neighbour with that IXP port private_interconnect
at one IXP
Upgrade: link +100Mb/s on one link with that link upgrades.link_capacity_mbps
capacity
Upgrade: buffer +12,500 bytes per port, one — upgrades.buffer_bytes_per_port
switch
Upgrade: faster port +200Mb/s on one IXP port with that port upgrades.port_mbps
Upgrade: program +128MiB in this world — upgrades.program_memory_bytes
memory
Repair crew your outages are half as long — repair_crew.outage_factor
Table 2: The hardware at a glance.

## Source PDF page 3

3 How an auction works
Every item in Section 2 is sold in an open auction: everyone sees the winning bids as they come in,
and a new bid must beat the ones already there.
What is for sale
• A lot is one kind of item from Section 2 in one world, for example the second cables in world 2.
It has a number of identical units and a reserve, its lowest price.
• Every team has a seat in every world, so every team may bid on every lot. The AI seats do
not bid in Auction 1. A lot is paid for from that world’s credits (Section 1).
• Each bid is for one unit. A team may hold winning bids for at most the lot’s per-team limit
of units (Section 5), counting joint bids it leads or joins.
Bidding
• Winning bids. At any moment the highest bids on a lot, one for each unit for sale, are
winning. The website shows every lot’s winning bids, with each bidder’s pseudonym, and its
minimum bid.
• The minimum bid. While a unit has no bid, it is the reserve. Once every unit has a bid, it
is the lowest winning bid raised by one step: the fraction auctions.min_raise of that bid,
rounded to whole credits (halves up), and at least 1cr. So after 10cr the minimum is 11cr;
after 20cr, 22cr; after 34cr, 37cr. You may bid any whole number of credits at or above the
minimum.
• Outbid. While a unit has no bid, a new bid takes it. Once every unit has a bid, a new bid
takes the unit of the lowest winning bid, which is then outbid. Of two equal winning bids, the
later one is outbid first.
• Your credits are held. When you bid, that many credits leave your credits in that world
at once, so you can never bid more than you have. If you are outbid, they come back at once
and you may bid again. A bid cannot be withdrawn.
The close
• Each lot closes on its own, at first at the auction’s closing time (schedule.auctions_close).
• Late bids push the close back, by less each time. A bid in the last e minutes before a
lot’s close moves that lot’s close e minutes later, and e halves. The first e is auctions.ext
ension_minutes, so the extensions are 10, 5, 2.5 and 1.25 minutes; once e would be under
a minute there are no more. Every lot therefore closes less than 20 minutes after the closing
time.
• At the close each winning bid wins one unit, and the team pays its own bid: the credits
already held. Units with no bid are not sold. Every bid and every outbid goes on the ledger
as it happens.
How to bid
• Decide your top price first: what the item is worth to you (Section 2). Never bid above
it.

## Source PDF page 4

A bids 10 B bids 12 C bids 11 A bids 13 D bids 13 B bids 14
Mon Mon Tue Wed 12:00 Wed 19:55 Wed 20:08
unit 1 A 10 A 10 C 11 A 13 A 13 A 13
unit 2 no bid B 12 B 12 B 12 D 13 B 14
minimum bid 10 11 12 13 14 14
lot closes 20:00 20:00 20:00 20:00 20:10 20:15
credits back A: 10 C: 11 B: 12 D: 13
Figure 1: An example with made-up numbers: one world’s lot of 2 second cables, reserve 10cr,
closing Wednesday at 20:00. Each column is one bid and the state just after it. C’s 11 outbids A’s
10, the lowest winning bid, and A’s 10cr come back; A then bids 13. D bids at 19:55, in the last 10
minutes, so the close moves to 20:10 and the next extension is 5 minutes. B’s 14 at 20:08 outbids
D (D’s 13 is later than A’s 13) and moves the close to 20:15. Nobody bids after that: A pays 13cr
and B 14cr; C and D pay nothing.
• Bid the minimum, and raise only when you are outbid. You pay your own bid, so
bidding more than you need to costs you the difference.
• Your winning bids hold your credits. Credits held on one lot cannot be spent on another,
so you may not be able to win everything. Choose the items you need most.
• You pay when you win, even for an item that needs someone else: a link across the ring
carries nothing until you have an agreement at the far IXP, and a private interconnect cannot
be installed until the neighbour agrees. Line these up before you bid.
• Watch the close. Late bids near the closing time push the close back, so the last few
minutes decide many lots.
Joint bids
• Several teams in the same world may bid together for one unit, with fixed shares of the cost,
in whole credits, that add up to the bid.
• One team, the lead, places the bid and names its partners and their shares. The lead’s credits
hold the whole bid until each partner confirms on the website; then that partner’s share is
held from its own credits instead, and the lead gets it back. A share that is not confirmed,
or is declined, stays on the lead, who pays it if the bid wins. If the bid is outbid, each team
gets back what it put in.
• The lead holds what is won and installs it on its own AS, so agree with your partners where
before you bid.
• A jointly bought private interconnect joins two partners’ hubs. If either partner complains
to us about how it is used, its capacity is split between their traffic in exactly the ratio each
paid.
4 After you buy
• Install. What you win goes to your inventory in that world. Install it on your own AS
there, choosing where (which link, switch or port, which site across the ring, which neighbour
for a private interconnect). It works from the next cutoff. A private interconnect needs its

## Source PDF page 5

neighbour to agree first. Installed hardware appears to your switch program at start-up like
the rest of your network; the API guide’s Section 6 says where, and how to add it to a practice
world to try it first.
• Move. You may move an item to another place in your AS; the move takes effect at the next
cutoff.
• Keep or sell. Hardware is yours for the rest of the game. You may sell it to another team
in the same world, for credits only (never for dollars or through an agreement), at any price
you both agree on; the sale goes on the ledger, and the item goes to the buyer’s inventory,
not installed. Sales that only move credits can be reversed (rule book, sham deals).
• What everyone sees. The map on the website shows every world’s installed hardware.
5 Auction 1
Auction 1 opens with the game, Mon Nov 9 at 18:00 (schedule.game_opens), and closes at the first
entry of schedule.auctions_close, or a little later for a lot with late bids (Section 3). Hardware
you win works from the next cutoff after you install it, so from Round 2 at the earliest. Its lots,
the same in each world, are in auctions.lots: for each kind, the units for sale per world, the
reserve and the most units one team may win. Auction 1 sells both kinds of link across the ring,
the second cable, the repair crew, the private interconnect, and the buffer and program-memory
upgrades. Lots for Auctions 2 and 3 are posted on the website the Monday before each one, and
bidding opens when they are posted.

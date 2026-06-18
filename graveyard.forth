\ GRAVEYARD MAP — SNAPKITTYWEST/ledge
\ 1 repos | rendered by AHMAD-BOT + Forth renderer
\ The graveyard in Forth. Every repo is a word.

\ ── ledge (gravity: 0.4, status: broken) ──
: crawl-ledge ( -- )
  0.4 gravity
  dup alive? IF
    ." ledge alive " cr
  ELSE dup broken? IF
    ." ledge broken " cr
    "ledge" repair
  ELSE
    ." ledge orphan " cr
    "ledge" flag
  THEN THEN
  drop
;

: crawl-graveyard ( -- )
  ." === SNAPKITTYWEST/ledge GRAVEYARD CRAWL ===" cr
  crawl-ledge
  ." === CRAWL COMPLETE ===" cr
;

crawl-graveyard
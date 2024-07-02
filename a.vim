let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/code/moa-payments
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +14 src/main.rs
badd +5 ~/code/moa-payments/diesel.toml
badd +13 ~/code/moa-payments/src/db/schema.rs
badd +3 ~/code/moa-payments/src/db/mod.rs
badd +10 ~/code/moa-payments/Cargo.toml
badd +19 ~/code/moa-payments/src/db/users.rs
badd +9 ~/code/moa-payments/src/db/error.rs
badd +18 ~/code/moa-payments/src/db/models.rs
badd +0 ~/code/moa-payments/migrations/2024-07-01-183726_create_users/up.sql
badd +25 ~/code/moa-payments/src/handlers/users.rs
argglobal
%argdel
set stal=2
tabnew +setlocal\ bufhidden=wipe
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit ~/code/moa-payments/src/db/models.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
wincmd _ | wincmd |
vsplit
2wincmd h
wincmd _ | wincmd |
split
1wincmd k
wincmd w
wincmd w
wincmd w
wincmd _ | wincmd |
split
1wincmd k
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 27 + 28) / 57)
exe 'vert 1resize ' . ((&columns * 91 + 137) / 274)
exe '2resize ' . ((&lines * 26 + 28) / 57)
exe 'vert 2resize ' . ((&columns * 91 + 137) / 274)
exe 'vert 3resize ' . ((&columns * 90 + 137) / 274)
exe '4resize ' . ((&lines * 27 + 28) / 57)
exe 'vert 4resize ' . ((&columns * 91 + 137) / 274)
exe '5resize ' . ((&lines * 26 + 28) / 57)
exe 'vert 5resize ' . ((&columns * 91 + 137) / 274)
argglobal
balt ~/code/moa-payments/src/db/schema.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 15 - ((14 * winheight(0) + 13) / 26)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 15
normal! 041|
wincmd w
argglobal
if bufexists(fnamemodify("~/code/moa-payments/src/db/schema.rs", ":p")) | buffer ~/code/moa-payments/src/db/schema.rs | else | edit ~/code/moa-payments/src/db/schema.rs | endif
if &buftype ==# 'terminal'
  silent file ~/code/moa-payments/src/db/schema.rs
endif
balt ~/code/moa-payments/src/db/models.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 13 - ((12 * winheight(0) + 12) / 25)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 13
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("~/code/moa-payments/src/db/users.rs", ":p")) | buffer ~/code/moa-payments/src/db/users.rs | else | edit ~/code/moa-payments/src/db/users.rs | endif
if &buftype ==# 'terminal'
  silent file ~/code/moa-payments/src/db/users.rs
endif
balt ~/code/moa-payments/src/db/error.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
4,8fold
15,16fold
15,17fold
15,18fold
15,19fold
11,20fold
10,21fold
let &fdl = &fdl
let s:l = 12 - ((11 * winheight(0) + 26) / 53)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 12
normal! 020|
wincmd w
argglobal
if bufexists(fnamemodify("~/code/moa-payments/src/db/mod.rs", ":p")) | buffer ~/code/moa-payments/src/db/mod.rs | else | edit ~/code/moa-payments/src/db/mod.rs | endif
if &buftype ==# 'terminal'
  silent file ~/code/moa-payments/src/db/mod.rs
endif
balt ~/code/moa-payments/src/db/users.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
8,14fold
16,18fold
23,24fold
23,25fold
21,27fold
29,31fold
20,32fold
let &fdl = &fdl
let s:l = 2 - ((0 * winheight(0) + 13) / 26)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 2
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("~/code/moa-payments/src/db/error.rs", ":p")) | buffer ~/code/moa-payments/src/db/error.rs | else | edit ~/code/moa-payments/src/db/error.rs | endif
if &buftype ==# 'terminal'
  silent file ~/code/moa-payments/src/db/error.rs
endif
balt ~/code/moa-payments/src/db/mod.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
5,10fold
let &fdl = &fdl
let s:l = 8 - ((7 * winheight(0) + 12) / 25)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 8
normal! 047|
wincmd w
exe '1resize ' . ((&lines * 27 + 28) / 57)
exe 'vert 1resize ' . ((&columns * 91 + 137) / 274)
exe '2resize ' . ((&lines * 26 + 28) / 57)
exe 'vert 2resize ' . ((&columns * 91 + 137) / 274)
exe 'vert 3resize ' . ((&columns * 90 + 137) / 274)
exe '4resize ' . ((&lines * 27 + 28) / 57)
exe 'vert 4resize ' . ((&columns * 91 + 137) / 274)
exe '5resize ' . ((&lines * 26 + 28) / 57)
exe 'vert 5resize ' . ((&columns * 91 + 137) / 274)
tabnext
edit ~/code/moa-payments/src/handlers/users.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
wincmd _ | wincmd |
vsplit
2wincmd h
wincmd w
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 91 + 137) / 274)
exe 'vert 2resize ' . ((&columns * 91 + 137) / 274)
exe 'vert 3resize ' . ((&columns * 90 + 137) / 274)
argglobal
balt ~/code/moa-payments/src/db/models.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
8,9fold
8,10fold
8,11fold
8,12fold
7,13fold
16,19fold
25,28fold
22,28fold
31,33fold
36,38fold
41,43fold
let &fdl = &fdl
let s:l = 25 - ((24 * winheight(0) + 26) / 53)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 25
normal! 020|
wincmd w
argglobal
if bufexists(fnamemodify("~/code/moa-payments/src/db/models.rs", ":p")) | buffer ~/code/moa-payments/src/db/models.rs | else | edit ~/code/moa-payments/src/db/models.rs | endif
if &buftype ==# 'terminal'
  silent file ~/code/moa-payments/src/db/models.rs
endif
balt ~/code/moa-payments/src/handlers/users.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
10,16fold
18,20fold
24,27fold
let &fdl = &fdl
let s:l = 18 - ((17 * winheight(0) + 26) / 53)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 18
normal! 06|
wincmd w
argglobal
if bufexists(fnamemodify("src/main.rs", ":p")) | buffer src/main.rs | else | edit src/main.rs | endif
if &buftype ==# 'terminal'
  silent file src/main.rs
endif
balt ~/code/moa-payments/src/handlers/users.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
14,15fold
14,16fold
13,17fold
13,18fold
13,19fold
9,21fold
let &fdl = &fdl
let s:l = 12 - ((11 * winheight(0) + 26) / 53)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 12
normal! 0
wincmd w
2wincmd w
exe 'vert 1resize ' . ((&columns * 91 + 137) / 274)
exe 'vert 2resize ' . ((&columns * 91 + 137) / 274)
exe 'vert 3resize ' . ((&columns * 90 + 137) / 274)
tabnext
edit ~/code/moa-payments/Cargo.toml
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 95 + 137) / 274)
exe 'vert 2resize ' . ((&columns * 178 + 137) / 274)
argglobal
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 10 - ((9 * winheight(0) + 26) / 53)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 10
normal! 062|
wincmd w
argglobal
if bufexists(fnamemodify("~/code/moa-payments/migrations/2024-07-01-183726_create_users/up.sql", ":p")) | buffer ~/code/moa-payments/migrations/2024-07-01-183726_create_users/up.sql | else | edit ~/code/moa-payments/migrations/2024-07-01-183726_create_users/up.sql | endif
if &buftype ==# 'terminal'
  silent file ~/code/moa-payments/migrations/2024-07-01-183726_create_users/up.sql
endif
balt ~/code/moa-payments/Cargo.toml
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=999
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
2,7fold
let &fdl = &fdl
let s:l = 8 - ((7 * winheight(0) + 26) / 53)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 8
normal! 0
wincmd w
exe 'vert 1resize ' . ((&columns * 95 + 137) / 274)
exe 'vert 2resize ' . ((&columns * 178 + 137) / 274)
tabnext 2
set stal=1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :

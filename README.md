<h2 align="center"> ━━━━━━  ❖  ━━━━━━ </h2>

<!-- BADGES -->
<div align="center">
   <p></p>
   
   <img src="https://img.shields.io/github/stars/dotzenith/lovesay.rs?color=F8BD96&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/forks/dotzenith/lovesay.rs?color=DDB6F2&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/repo-size/dotzenith/lovesay.rs?color=ABE9B3&labelColor=302D41&style=for-the-badge">
   
   <img src="https://img.shields.io/github/commit-activity/y/dotzenith/lovesay.rs?color=96CDFB&labelColor=302D41&style=for-the-badge&label=COMMITS"/>
   <br>
</div>

<p/>

---

### ❖ Information 

  lovesay.rs is a port of [lovesay](https://github.com/dotzenith/lovesay) in rust. It is a simple rust program that displays a quote from a loved one based on the day of the month or a quote passed in through the cli arguments. 

  <img src="https://github.com/dotzenith/dotzenith/blob/main/assets/lovesayrs/lovesay.png" alt="lovesay crab">

---

### ❖ Requirements

Note: These requirements only apply if using you're using lovesay to print a different quote for each day of the month.  

- A quotes file stored in `$HOME/.config/lovesay/`
- Each quote must be on a new line, see the example quotes file in `.example/quotes`
- (optional) A partner to write you 31 lines full of love, one for each day of the month

---

### ❖ Installation

> Install from pip
```sh
cargo install lovesay
```

> Install from source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/lovesay.rs.git
cd lovesay.rs
cargo build --release
./target/release/lovesay
```

### ❖ Usage 

lovesay can be used in a similar fashion to cowsay

```sh
lovesay Hello World
```

if you're not using a nerd-font, be sure to call lovesay with the `--no-nerd` flag
```sh
lovesay --no-nerd hello world
```
> NOTE: `--no-nerd` has to be the first argument if you're using it because I didn't feel like implementing a proper CLI interface yet

if there's a `quotes` file in `$HOME/.config/lovesay/`, lovesay can be used without any arguments

```sh
lovesay
```

if you'd like to use a quotes stored somewhere other than the path above, the `LOVESAY_PATH` env variable can be used as such

```sh
export LOVESAY_PATH="~/path/to/file"
```

lovesay also supports pipes

```sh
lovesay | lolcat                    # pipe output to lolcat

echo something | lovesay            # take input from another command

echo something | lovesay | lolcat   # combine the two
```


---

### ❖ About lovesay

This is just a silly little rust port, but the reasoning for the original is below:

I wrote lovesay because I got tired of seeing neofetch or pfetch every time I opened my terminal. I wanted something more personal. 

Seeing words full of love from my partner is a lot better than any other command I could possibly run. It makes my terminal feel cozy, welcoming, and as is the case with most things my partner touches, it makes my terminal feel like home. 

I hope that someone else finds a use for this little script as well. Love is a wonderful thing, and we could all use a little bit more of it in our lives (especially arch linux users)

---

### ❖ What's New? 
0.3.0 - Stop discriminating against non-nerds

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>


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

  <img src="https://assets.danshu.co/lovesayrs/lovesay.gif" alt="lovesay output in different colorschemes">

---

### ❖ Requirements

Note: These requirements only apply if using you're using lovesay to print a different quote for each day of the month.  

- A quotes file stored in `~/.config/lovesay/`
- Each quote must be on a new line, see the example quotes file in `.example/quotes`
- (optional) A partner to write you 31 lines full of love, one for each day of the month

---

### ❖ Installation

#### Shell
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dotzenith/lovesay.rs/releases/latest/download/lovesay-installer.sh | sh
```

#### Brew
```sh
brew install dotzenith/tap/lovesay
```

#### Powershell
```sh
powershell -ExecutionPolicy ByPass -c "irm https://github.com/dotzenith/lovesay.rs/releases/latest/download/lovesay-installer.ps1 | iex"
```

#### Cargo
```sh
cargo install lovesay
```

#### Binaries
Pre-Compiled binaries for linux, mac, and windows are available in [Releases](https://github.com/dotzenith/lovesay.rs/releases)

#### Source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/lovesay.rs.git
cd lovesay.rs
cargo build --release
./target/release/lovesay
```

### ❖ Usage 

#### lovesay can be used in a similar fashion to cowsay

```sh
lovesay Hello World
```

#### if there's a `quotes` file in `~/.config/lovesay/`, lovesay can be used without any arguments

```sh
lovesay
```

#### if you'd like to use a quotes stored somewhere other than the path above, the `LOVESAY_PATH` env variable can be used as such

```sh
export LOVESAY_PATH="~/path/to/file"
```

#### Colorschemes

lovesay uses your terminal's colorscheme by default, but
other colorschemes can be set using an env variable

```sh
export LOVESAY_COLORSCHEME=nord # uses catppuccin mocha by default if this is left blank or malformed
```

The available colorschemes are as follows: 

- catppuccin latte
- catppuccin frappe
- catppuccin macchiato
- catppuccin mocha
- dracula
- nord
- gruvbox dark
- gruvbox light
- onedark
- tokyonight
- ayu
- palenight
- gogh
- biscuit dark
- biscuit light

#### lovesay also supports two other environment variables

```sh
export LOVESAY_NO_NERD=1        # Set if your terminal does not use a nerd font (can be set to anything, lovesay just checks for existence)

export LOVESAY_MAX_WIDTH=80     # Override the width lovesay will use to wrap a quote
```

#### lovesay has support for pipes as well

```sh
lovesay | lolcat                    # pipe output to lolcat

echo something | lovesay            # take input from another command

echo something | lovesay | lolcat   # combine the two
```

---

### ❖ About lovesay

While this rust port has grown up to be the main version, the reasoning for the original is below:

I wrote lovesay because I got tired of seeing neofetch or pfetch every time I opened my terminal. I wanted something more personal. 

Seeing words full of love from my partner is a lot better than any other command I could possibly run. It makes my terminal feel cozy, welcoming, and as is the case with most things my partner touches, it makes my terminal feel like home. 

I hope that someone else finds a use for this little script as well. Love is a wonderful thing, and we could all use a little bit more of it in our lives (especially arch linux users)

---

### ❖ What's New? 

1.0.2 - Update Rust edition

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>


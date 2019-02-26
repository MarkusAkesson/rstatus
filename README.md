# rstatus
dwm statusbar written in rust


# Build
While standing in the rstatus directory:

`cargo build`

# Install 
While standing in the rstatus directory:
`cargo install --path .`

# Usage

`while true; do
  xsetroot -name "$(rstatus)"
  sleep 1m # update every minute
done & `

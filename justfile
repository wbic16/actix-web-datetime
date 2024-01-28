# just commands for actix-web-datetime
# note: these only work in Powershell

set shell := ["powershell", "-c"]

run-server:
  cd backend; cargo run

run-front:
  cd frontend; trunk serve --open

class Rocks < Formula
  desc "Interpreter for rocks-lang"
  homepage "https://github.com/boranseckin/rocks"
  url "https://github.com/boranseckin/rocks/releases/latest/download/rocks.tar.gz"
  version "0.1.1"
  sha256 "78d9d05ca5c0573ff532fc88356f10cb5638ae5c2ef06df12158caa20ad90fde"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "cargo", "test", *std_cargo_args
  end
end

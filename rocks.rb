class Rocks < Formula
  desc "Interpreter for rocks-lang"
  homepage "https://github.com/boranseckin/rocks"
  url "https://github.com/boranseckin/rocks/releases/latest/download/rocks.tar.gz"
  version "0.1.1"
  sha256 "e8da60f59822bbe71bab71f4bb39b05f67f369b6e52f000e5773c93158268f24"
  license "MIT"

  def install
    bin.install "rocks"
  end
end

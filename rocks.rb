class Rocks < Formula
  desc "Interpreter for rocks-lang"
  homepage "https://github.com/boranseckin/rocks"
  url "https://github.com/boranseckin/rocks/releases/latest/download/rocks.tar.gz"
  version "0.2.1"
  sha256 "25cf524709bf7d562dba84fe727c9ea994a73c1877121f966e304b676f778c12"
  license "MIT"

  def install
    bin.install "rocks"
  end
end

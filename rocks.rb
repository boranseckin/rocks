class Rocks < Formula
  desc "Interpreter for rocks-lang"
  homepage "https://github.com/boranseckin/rocks"
  url "https://github.com/boranseckin/rocks/releases/latest/download/rocks.tar.gz"
  version "0.2.0"
  sha256 "4da60c028f00a9275adc1d9ef7b40e3d3db827cd27a5c3c638991bbe0e2a9b55"
  license "MIT"

  def install
    bin.install "rocks"
  end
end

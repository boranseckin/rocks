class Rocks < Formula
  desc "Interpreter for rocks-lang"
  homepage "https://github.com/boranseckin/rocks"
  url "https://github.com/boranseckin/rocks/releases/latest/download/rocks.tar.gz"
  version "0.2.2"
  sha256 "744c57783ba96999c34a47cf99b18814571a1ed823a398efb26a0d79cc49becf"
  license "MIT"

  def install
    bin.install "rocks"
  end
end

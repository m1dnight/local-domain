defmodule LocalDomainTest do
  use ExUnit.Case
  doctest LocalDomain

  test "greets the world" do
    assert LocalDomain.hello() == :world
  end
end

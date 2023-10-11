# frozen_string_literal: true

require_relative "regox/version"
require 'rutie'

class RegoxRutie
  Rutie.new(:regox).init("Init_regox_regex", __dir__)
end

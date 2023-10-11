#!/usr/bin/env ruby

require_relative 'lib/regox'

require 'benchmark/ips'

HAYSTACK = File.read('test-file')
PATTERN = %q{'(data:image/jpeg;base64,[a-zA-Z0-9/+\\\]+)';var ii=\['([^']+)'\]}

# HAYSTACK = ('a' * 4096) + 'z'
# PATTERN = '^a*b?a*$'

regex = Regexp.new(PATTERN)
regox = Regox.new(PATTERN)

a = HAYSTACK.scan(regex)
b = regox.scan(HAYSTACK)

p a.size
raise "Differ" if a != b

Benchmark.ips do |x|
  x.report("Regex match?") { regex.match?(HAYSTACK) }
  x.report("Regox match?") { regox.match?(HAYSTACK) }
  x.compare!
end

Benchmark.ips do |x|
  x.report("Regex scan") { HAYSTACK.scan(regex) }
  x.report("Regox scan") { regox.scan(HAYSTACK) }
  x.compare!
end

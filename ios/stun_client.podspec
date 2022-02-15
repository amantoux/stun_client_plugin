#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint stun_client.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name                = 'stun_client'
  s.version             = '0.0.1'
  s.summary             = 'A simple STUN client.'
  s.description         = <<-DESC
  A simple STUN client.
                       DESC
  s.license             = { :file => '../LICENSE' }
  s.author              = { 'Plato' => 'alan.mantoux@gmail.com' }
  s.source              = { :path => '.' }
  s.homepage            = "plato.mantoux.org"
  s.public_header_files = 'Classes**/*.h'
  s.source_files        = 'Classes/**/*'
  s.static_framework    = true
  s.vendored_libraries  = "**/*.a"
  s.dependency 'Flutter'
  s.platform = :ios, '9.0'

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version = '5.0'
end

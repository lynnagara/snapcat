#!/usr/bin/env python

from setuptools import setup

def build_native(spec):
  build = spec.add_external_build(
      cmd=['cargo', 'build', '--release'],
      path='./rust'
  )

  spec.add_cffi_module(
      module_path='snapcat._native',
      dylib=lambda: build.find_dylib('snapcat', in_path='target/release'),
      header_filename=lambda: build.find_header('snapcat.h', in_path='target'),
      rtld_flags=['NOW', 'NODELETE']
  )


setup(name='snapcat',
  version='0.1',
  description='Diff images of cats',
  url='http://github.com/lynnagara/snapcat',
  author='Snaps',
  author_email='lyn.nagara@gmail.com',
  license='MIT',
  packages=['snapcat'],
  zip_safe=False,
  setup_requires=['milksnake'],
  install_requires=['milksnake'],
  milksnake_tasks=[
      build_native
  ]  
)

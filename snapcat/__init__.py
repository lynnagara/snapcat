from . import _native

BASE_PATH = '.snapshots/'

import re
import os
import subprocess

class Snapcat():
  def __init__(self):
    with open('.git/HEAD', 'r') as f:
      ref = re.sub(r'^ref: ', '', f.readline()).rstrip()
      self.path = BASE_PATH + sanitize_file_path(ref) + '/'

  def take(self, save, name):
    file_path = '{}{}.png'.format(self.path, name)
    ensure_directory_exists(self.path)
    save(file_path)

  def compare(self):
    _native.lib.generate_diffs(0.1)

def sanitize_file_path(path):
  sanitize = path.replace('/', '-')
  return sanitize


def ensure_directory_exists(path):
  dir = os.path.dirname(path)
  if not os.path.exists(dir):
    os.makedirs(path)

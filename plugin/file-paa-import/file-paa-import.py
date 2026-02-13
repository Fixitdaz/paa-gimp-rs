#!/usr/bin/env python3
# author: fixitdaztv
# date: 2026-02-13

import gi
gi.require_version('Gimp', '3.0')
from gi.repository import Gimp, GObject, Gio, GLib
import os
import sys
import subprocess
import tempfile

# Users must update this path to point to their compiled .exe
CONVERTER_PATH = r"D:\fixit\Documents\paa-gimp-rs\target\debug\paa-gimp-rs.exe"

class PaaImport(Gimp.PlugIn):
    def do_query_procedures(self):
        return ["file-paa-load"]

    def do_create_procedure(self, name):
        procedure = Gimp.LoadProcedure.new(self, name, Gimp.PDBProcType.PLUGIN, self.run)
        procedure.set_menu_label("Arma PAA Texture")
        procedure.set_documentation("Loads Arma PAA files via Rust",
                                    "Converts PAA to PNG and loads into GIMP",
                                    name)
        procedure.set_attribution("fixitdaztv", "fixitdaztv", "2026")
        procedure.set_extensions("paa")
        return procedure

    def run(self, procedure, config, data):
        gfile = config.get_property('file')
        if not gfile:
            return procedure.new_return_values(Gimp.PDBStatusType.CALL_ERROR, GLib.Error())

        filename = gfile.get_path()
        temp_png = os.path.join(tempfile.gettempdir(), "gimp_paa_temp.png")

        try:
            # Execute the Rust converter
            subprocess.check_call([CONVERTER_PATH, "--input", filename, "--output", temp_png])

            pdb = Gimp.get_pdb()
            result = pdb.run_procedure('file-png-load', [
                GObject.Value(Gio.File, Gio.File.new_for_path(temp_png)),
            ])
            
            image = result.index(1)

            if os.path.exists(temp_png):
                os.remove(temp_png)

            return_vals = procedure.new_return_values(Gimp.PDBStatusType.SUCCESS, None)
            return_vals.remove(1)
            return_vals.insert(1, GObject.Value(Gimp.Image, image))
            return return_vals

        except Exception as e:
            print(f"PAA Plugin Error: {e}")
            return procedure.new_return_values(Gimp.PDBStatusType.EXECUTION_ERROR, None)

if __name__ == "__main__":
    Gimp.main(PaaImport.__gtype__, sys.argv)

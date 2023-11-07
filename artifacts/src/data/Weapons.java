package data;

import java.util.Arrays;
import java.util.List;

public class Weapons {
    static List<String> weapons = Arrays.asList(
            "miecz",
            "tasak",
            "sztylet",
            "młot",
            "maczuga",
            "topór",
            "nóż",
            "rapier",
            "kastet",
            "tomahawk",
            "ostrze",
            "hak",
            "kosa",
            "halabarda",
            "włócznia",
            "kostur",
            "różdżka",
            "proca",
            "łuk",
            "kusza",
            "kopia",
            "morgenstern",
            "lanca",
            "szpada",
            "arkan",
            "bicz",
            "nóż do rzucania",
            "topór do rzucania",
            "oszczep",
            "tarcza",
            "puklerz" );

    static List<String> prefixes = Arrays.asList(
            "wyrafinowan",
            "specjaln",
            "mistrzowsk",
            "egzotyczn",
            "żelazn",
            "srebrny",
            "stalow",
            "złot",
            "ognist",
            "platynow",
            "miedzian",
            "mahakamsk",
            "krasnoludzk",
            "elfick",
            "smocz",
            "goblińsk",
            "górsk",
            "ogromn",
            "lśniąc",
            "magiczn",
            "ciężk",
            "lekk",
            "zaklęt",
            "zatrut",
            "dług",
            "krótk",
            "runiczn",
            "jednoręczn",
            "dwuręczn",
            "krwist",
            "ceremonialn",
            "tajemn",
            "czarodziejsk"
    );

    public static List<String> getWeapons() {
        return weapons;
    }

    public static List<String> getPrefixes(){
        return prefixes;
    }
}

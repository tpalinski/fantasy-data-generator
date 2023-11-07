package data;

import java.util.Arrays;
import java.util.List;

public class Armors {
    static List<String> armors = Arrays.asList(
            "obuwie",
            "nagolenniki",
            "sandały",
            "buty",
            "mokasyny",
            "kamasze",
            "pantofle",
            "kozaki",
            "spodnie",
            "karawasze",
            "rękawice",
            "hełm",
            "maska",
            "kapelusz",
            "kaptur",
            "czepiec",
            "pas",
            "zbroja",
            "pancerz",
            "kolczuga",
            "szata",
            "frak",
            "kamizelka",
            "kubrak",
            "napierśnik",
            "opończa"
    );

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
            "skórzan",
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
            "runiczn",
            "krwist",
            "ceremonialn",
            "tajemn",
            "czarodziejsk"
    );

    public static List<String> getArmors() {
        return armors;
    }

    public static List<String> getPrefixes() {
        return prefixes;
    }
}

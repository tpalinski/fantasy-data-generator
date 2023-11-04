import data.Armors;
import data.Others;
import data.Weapons;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Main {

    public static void main(String[] args) throws IOException {
        Generator generator = new Generator();
        generator.saveItems(Others.getOthers(), Others.getPrefixes());
        generator.saveItems(Weapons.getWeapons(), Weapons.getPrefixes());
        generator.saveItems(Armors.getArmors(), Armors.getPrefixes());
    }



}
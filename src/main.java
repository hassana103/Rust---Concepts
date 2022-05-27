import java.util.Scanner;

public class main {

    public static void main(String[] args){
        
        //getting input
        Scanner in = new Scanner(System.in);

        //getting first number n
        int n = Integer.parseInt(in.nextLine());

        //getting fs array (2n)
        char[] fs=in.nextLine().toCharArray();

        boolean dum = false;

        for(int i=0 ; i<=n ; i++){
            if (fs[i] == fs[n+i]){
                System.out.println("NO");
                return;
            }
        }
        System.out.println("YES");

    }

}